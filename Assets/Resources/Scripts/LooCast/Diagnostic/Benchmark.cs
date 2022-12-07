using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Diagnostic
{
    public class Benchmark
    {
        #region Properties
        public string Name => name;
        public TimeSpan LatestDuration => latestDuration;
        public TimeSpan DurationSum => durationSum;
        public TimeSpan AverageDuration => averageDuration;
        public TimeSpan MaxDuration => maxDuration;
        public TimeSpan MinDuration => minDuration;
        #endregion

        #region Static Fields
        private static Dictionary<string, Benchmark> benchmarks = new Dictionary<string, Benchmark>();
        #endregion

        #region Fields
        private string name;
        private int samples;
        private bool running;
        private DateTime startTime;
        private DateTime endTime;
        private TimeSpan latestDuration;
        private TimeSpan durationSum;
        private TimeSpan averageDuration;
        private TimeSpan maxDuration;
        private TimeSpan minDuration;
        #endregion

        #region Constructors
        private Benchmark(string name)
        {
            this.name = name;
            samples = 0;
            running = false;
        }
        #endregion

        #region Static Methods
        public static Benchmark Create(string name)
        {
            if (benchmarks.ContainsKey(name))
            {
                throw new Exception($"ID {name} already exists!");
            }
            
            Benchmark benchmark = new Benchmark(name);
            benchmarks.Add(name, benchmark);
            return benchmark;
        }

        public static void Start(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"ID {name} does not exist!");
            }

            benchmarks[name].Start();
        }

        public static void Stop(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"ID {name} does not exist!");
            }

            benchmarks[name].Stop();
        }
        #endregion

        #region Methods
        private void Start()
        {
            if (running)
            {
                throw new Exception("Benchmark is already started!");
            }
            
            running = true;
            startTime = DateTime.Now;
        }

        private void Stop()
        {
            if (!running)
            {
                throw new Exception("Benchmark is already stopped!");
            }

            running = false;
            endTime = DateTime.Now;
            samples++;
            latestDuration = endTime - startTime;
            durationSum += latestDuration;
            averageDuration = durationSum / samples;
            if (maxDuration < latestDuration)
            {
                maxDuration = latestDuration;
            }
            if (minDuration > latestDuration)
            {
                minDuration = latestDuration;
            }
        }
        #endregion
    }
}
