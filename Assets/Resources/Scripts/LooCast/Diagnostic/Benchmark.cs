using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Diagnostic
{
    public class Benchmark
    {
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
        public static void Create(string name)
        {
            if (benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' already exists!");
            }
            
            Benchmark benchmark = new Benchmark(name);
            benchmarks.Add(name, benchmark);
        }

        public static void Delete(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            benchmarks.Remove(name);
        }

        public static void Start(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            benchmarks[name].Start();
        }

        public static void Stop(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            benchmarks[name].Stop();
        }

        public static TimeSpan LatestDuration(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            return benchmarks[name].latestDuration;
        }

        public static TimeSpan DurationSum(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            return benchmarks[name].durationSum;
        }

        public static TimeSpan AverageDuration(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            return benchmarks[name].averageDuration;
        }

        public static TimeSpan MaxDuration(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            return benchmarks[name].maxDuration;
        }

        public static TimeSpan MinDuration(string name)
        {
            if (!benchmarks.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            return benchmarks[name].minDuration;
        }
        #endregion

        #region Methods
        private void Start()
        {
            if (running)
            {
                throw new Exception($"Benchmark '{name}' is already started!");
            }
            
            running = true;
            startTime = DateTime.Now;
        }

        private void Stop()
        {
            if (!running)
            {
                throw new Exception($"Benchmark '{name}' is already stopped!");
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
