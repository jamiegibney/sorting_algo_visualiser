use super::*;
use crate::thread_pool::PoolCreationError;
use crossbeam_channel as cc;
use parking_lot::Mutex;
use std::{
    io::{Error, Result as IoResult},
    panic,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
        Arc,
    },
    thread::{self, JoinHandle},
};
use thread_priority as priority;

/// The maximum block size for audio processing.
pub const MAX_BLOCK_SIZE: usize = 64;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

#[derive(Debug)]
struct VoiceThreadData {
    /// The audio output buffer for this thread.
    output_buffer: Arc<Mutex<Vec<f32>>>,

    /// The note event receiver.
    note_receiver: Arc<Receiver<NoteEvent>>,

    /// The voice handler for this thread.
    voice_handler: Arc<Mutex<VoiceHandler>>,
    /// The active voice counter for this thread.
    voice_counter: Arc<AtomicU32>,
    /// Whether this thread's buffer has been modified.
    modified_flag: Arc<AtomicBool>,

    /// Whether this thread is currently busy.
    busy_flag: Arc<AtomicBool>,
    /// The number of tasks which are "queued".
    queue_count: Arc<AtomicU32>,
    /// The receiver to compute audio on this thread.
    execute_receiver: Receiver<()>,
}

impl Worker {
    fn new(id: usize, data: VoiceThreadData) -> IoResult<Self> {
        let builder = thread::Builder::new();

        let thread = builder.name(format!("audio voice thread #{id}")).spawn(
            move || {
                priority::set_current_thread_priority(
                    priority::ThreadPriority::Max,
                );

                let mut handler = data.voice_handler.lock();

                loop {
                    data.busy_flag.store(false, Relaxed);

                    if data.execute_receiver.recv().is_err() {
                        break;
                    }

                    data.busy_flag.store(true, Relaxed);
                    data.queue_count.fetch_sub(1, Relaxed);

                    'process: {
                        let mut next_event = data.note_receiver.try_recv().ok();
                        let mut buf = data
                            .output_buffer
                            .try_lock()
                            .unwrap_or_else(|| {
                                panic!(
                                    "failed to lock output buffer for thread {id}"
                                )
                            });

                        let buffer_len = buf.len() / NUM_CHANNELS;
                        buf.fill(0.0);

                        // TODO: could this be implemented?
                        // if next_event.is_none() && !handler.any_active() {
                        //     break 'process;
                        // }

                        data.modified_flag.store(true, Relaxed);

                        let mut block_start = 0;
                        let mut block_end = MAX_BLOCK_SIZE.min(buffer_len);

                        // handle polyphonic voices
                        while block_start < buffer_len {
                            'events: loop {
                                if handler.num_active() == VOICES_PER_HANDLER {
                                    break 'events;
                                }

                                match next_event {
                                    // if we've snapped the block to an event
                                    Some(event)
                                        if (event.sample_offset() as usize)
                                            <= block_start =>
                                    {
                                        handler.new_voice(event);
                                        next_event =
                                            data.note_receiver.try_recv().ok();
                                    }
                                    // if an event is within this block, snap to
                                    // the event
                                    Some(event)
                                        if (event.sample_offset() as usize)
                                            < block_end =>
                                    {
                                        block_end =
                                            event.sample_offset() as usize;
                                        break 'events;
                                    }
                                    // if no new events are available
                                    _ => break 'events,
                                }
                            }

                            // TODO: handle the voice gain in a better way.
                            let mut gain = [0.08; MAX_BLOCK_SIZE];

                            // process voices and clean any which are finished
                            handler.process_block(
                                &mut buf, block_start, block_end, gain,
                            );

                            handler.free_finished_voices();

                            block_start = block_end;
                            block_end =
                                (block_end + MAX_BLOCK_SIZE).min(buffer_len);
                        }

                        data.voice_counter
                            .store(handler.num_active() as u32, Relaxed);

                        // println!("processed voice from thread {id}");

                        drop(buf);
                        // println!("thread {id} dropped buffer");
                    }
                }

                drop(handler);
            },
        )?;

        Ok(Self { id, thread: Some(thread) })
    }

    fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

#[derive(Debug)]
pub struct AudioThreadPoolReferences<'a> {
    pub output_buffers: &'a [Arc<Mutex<Vec<f32>>>],
    pub voice_handlers: &'a [Arc<Mutex<VoiceHandler>>],
    pub voice_counters: &'a [Arc<AtomicU32>],
    pub modified_flags: &'a [Arc<AtomicBool>],
}

#[derive(Debug)]
pub struct AudioThreadPool {
    workers: Vec<Worker>,
    num_queued: Arc<AtomicU32>,
    busy_flags: Vec<Arc<AtomicBool>>,

    execute_senders: Vec<Option<Sender<()>>>,

    voice_counters: Vec<Arc<AtomicU32>>,
    note_receiver: Arc<Receiver<NoteEvent>>,
}

impl AudioThreadPool {
    pub fn build(
        refs: &AudioThreadPoolReferences<'_>,
        note_receiver: &Arc<Receiver<NoteEvent>>,
    ) -> Result<Self, PoolCreationError> {
        let mut workers = Vec::with_capacity(NUM_AUDIO_THREADS);
        let mut execute_senders = Vec::with_capacity(NUM_AUDIO_THREADS);
        let busy_flags: Vec<Arc<AtomicBool>> = (0..NUM_AUDIO_THREADS)
            .map(|_| Arc::new(AtomicBool::new(false)))
            .collect();
        let num_queued = Arc::new(AtomicU32::new(0));

        for id in 0..NUM_AUDIO_THREADS {
            let (execute_tx, execute_rx) = cc::bounded(0);
            execute_senders.push(Some(execute_tx));

            match Worker::new(id, VoiceThreadData {
                output_buffer: Arc::clone(&refs.output_buffers[id]),

                note_receiver: Arc::clone(note_receiver),

                voice_handler: Arc::clone(&refs.voice_handlers[id]),
                voice_counter: Arc::clone(&refs.voice_counters[id]),
                modified_flag: Arc::clone(&refs.modified_flags[id]),

                busy_flag: Arc::clone(&busy_flags[id]),
                execute_receiver: execute_rx,
                queue_count: Arc::clone(&num_queued),
            }) {
                Ok(worker) => workers.push(worker),
                Err(e) => return Err(PoolCreationError::FailedSpawn(e)),
            }
        }

        Ok(Self {
            workers,
            num_queued,
            busy_flags,

            execute_senders,

            voice_counters: refs
                .voice_counters
                .iter()
                .map(Arc::clone)
                .collect(),
            note_receiver: Arc::clone(note_receiver),
        })
    }

    /// This signals the pool's audio threads to compute voices. Returns `true`
    /// if at least one thread was signalled to compute, and false if none were
    /// signalled.
    ///
    /// This also modifies its attached modified flags, which can be used to
    /// identify which audio buffers have been modified.
    pub fn execute(&self) -> bool {
        // TODO: have channels for each thread, and divide the incoming events
        // amongst all threads. this would allow free threads to focus on new
        // voices, etc.

        let num_active_voices = self
            .voice_counters
            .iter()
            .map(|c| c.load(Relaxed))
            .sum::<u32>();
        let num_incoming = self.note_receiver.len() as u32;

        // if there are no incoming events and no active voices, don't do
        // anything.
        if num_incoming == 0 && num_active_voices == 0 {
            return false;
        }

        // otherwise, dispatch the minimum number of required threads to process
        // all the voices.
        for i in 0..(NUM_AUDIO_THREADS) {
            self.execute_thread(i);
        }

        self.block_until_free();

        true
    }

    /// Blocks the calling thread until all audio threads are free (i.e. when
    /// all audio processing is done).
    #[inline]
    pub fn block_until_free(&self) {
        // TODO: could a spin lock or yield be used here to improve efficiency?
        while self.any_busy() {}
    }

    /// Whether any of the audio threads are currently busy.
    pub fn any_busy(&self) -> bool {
        self.num_queued.load(Relaxed) > 0
            || self.busy_flags.iter().any(|b| b.load(Relaxed))
    }

    fn execute_thread(&self, thread_id: usize) {
        assert!(thread_id < NUM_AUDIO_THREADS);

        if let Some(tx) = &self.execute_senders[thread_id] {
            tx.send(()).unwrap();
            self.num_queued.fetch_add(1, Relaxed);
        }
    }
}

impl Drop for AudioThreadPool {
    fn drop(&mut self) {
        self.execute_senders
            .iter_mut()
            .for_each(|tx| drop(tx.take()));

        self.workers.iter_mut().for_each(Worker::join);
    }
}
