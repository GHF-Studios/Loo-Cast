using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using UnityEngine;

namespace LooCast.Diagnostic
{
    [Serializable]
    public class Benchmark
    {
        #region Structs
        [Serializable]
        public struct SerializableTimeSpan
        {
            #region Properties
            public double Seconds => seconds;
            public double Milliseconds => milliseconds;
            public double Microseconds => microseconds;
            #endregion

            #region Fields
            [SerializeField] private double seconds;
            [SerializeField] private double milliseconds;
            [SerializeField] private double microseconds;
            #endregion

            #region Constructors
            public SerializableTimeSpan(TimeSpan timeSpan)
            {
                seconds = timeSpan.TotalSeconds;
                milliseconds = timeSpan.TotalMilliseconds;
                microseconds = timeSpan.TotalMilliseconds * 1000;
            }
            #endregion
        }
        #endregion
        
        #region Static Fields
        private static ConcurrentDictionary<string, ConcurrentDictionary<int, Benchmark>> benchmarkDictionaries = new ConcurrentDictionary<string, ConcurrentDictionary<int, Benchmark>>();
        #endregion

        #region Fields
        [SerializeField] private string name;
        [SerializeField] private int threadID;
        [SerializeField] private int samples;
        [SerializeField] private SerializableTimeSpan latestDurationSerializable;
        [SerializeField] private SerializableTimeSpan durationSumSerializable;
        [SerializeField] private SerializableTimeSpan averageDurationSerializable;
        [SerializeField] private SerializableTimeSpan maxDurationSerializable;
        [SerializeField] private SerializableTimeSpan minDurationSerializable;
        private bool running;
        private Stopwatch stopwatch;
        private TimeSpan latestDuration;
        private TimeSpan durationSum;
        private TimeSpan averageDuration;
        private TimeSpan maxDuration;
        private TimeSpan minDuration;
        #endregion

        #region Constructors
        private Benchmark(string name, int threadID)
        {
            this.name = name;
            this.threadID = threadID;
            samples = 0;
            running = false;
            stopwatch = new Stopwatch();
            latestDuration = TimeSpan.Zero;
            durationSum = TimeSpan.Zero;
            averageDuration = TimeSpan.Zero;
            maxDuration = TimeSpan.Zero;
            minDuration = TimeSpan.MaxValue;
            latestDurationSerializable = new SerializableTimeSpan(latestDuration);
            durationSumSerializable = new SerializableTimeSpan(durationSum);
            averageDurationSerializable = new SerializableTimeSpan(averageDuration);
            maxDurationSerializable = new SerializableTimeSpan(maxDuration);
            minDurationSerializable = new SerializableTimeSpan(minDuration);
        }
        #endregion

        #region Static Methods
        public static void Create(string name)
        {
            if (benchmarkDictionaries.ContainsKey(name))
            {
                throw new ArgumentException($"A benchmark with the name '{name}' already exists.");
            }

            benchmarkDictionaries.TryAdd(name, new ConcurrentDictionary<int, Benchmark>());
        }

        public static void Delete(string name)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            benchmarkDictionaries.TryRemove(name, out _);
        }

        public static void Delete(string name, int threadID)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                throw new Exception($"Benchmark '{name}' with threadID '{threadID}' does not exist!");
            }

            benchmarkDictionaries[name].TryRemove(threadID, out _);
        }

        public static void Start(string name, int threadID = 0)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                benchmarkDictionaries[name].TryAdd(threadID, new Benchmark(name, threadID));
            }

            benchmarkDictionaries[name][threadID].Start();
        }

        public static void Stop(string name, int threadID = 0)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                throw new Exception($"Benchmark '{name}' with threadID '{threadID}' does not exist!");
            }

            benchmarkDictionaries[name][threadID].Stop();
        }

        public static TimeSpan LatestDuration(string name, int threadID)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                throw new Exception($"Benchmark '{name}' with threadID '{threadID}' does not exist!");
            }

            return benchmarkDictionaries[name][threadID].latestDuration;
        }

        public static TimeSpan LatestDuration(string name)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            return benchmarkDictionaries[name].ElementAt(benchmarkDictionaries[name].Count - 1).Value.latestDuration;
        }

        public static TimeSpan DurationSum(string name, int threadID)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                throw new Exception($"Benchmark '{name}' with threadID '{threadID}' does not exist!");
            }

            return benchmarkDictionaries[name][threadID].durationSum;
        }

        public static TimeSpan DurationSum(string name)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            TimeSpan durationSum = TimeSpan.Zero;
            Parallel.ForEach(benchmarkDictionaries[name], (benchmarkKeyValuePair) =>
            {
                durationSum += benchmarkKeyValuePair.Value.durationSum;
            });
            return durationSum;
        }

        public static TimeSpan AverageDuration(string name, int threadID)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                throw new Exception($"Benchmark '{name}' with threadID '{threadID}' does not exist!");
            }

            return benchmarkDictionaries[name][threadID].averageDuration;
        }

        public static TimeSpan AverageDuration(string name)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (benchmarkDictionaries[name].IsEmpty)
            {
                return TimeSpan.Zero;
            }

            TimeSpan averageDuration = TimeSpan.Zero;
            Parallel.ForEach(benchmarkDictionaries[name], (benchmarkKeyValuePair) =>
            {
                averageDuration += benchmarkKeyValuePair.Value.averageDuration;
            });
            averageDuration /= benchmarkDictionaries[name].Values.Count;
            return averageDuration;
        }

        public static TimeSpan MaxDuration(string name, int threadID)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                throw new Exception($"Benchmark '{name}' with threadID '{threadID}' does not exist!");
            }

            return benchmarkDictionaries[name][threadID].maxDuration;
        }

        public static TimeSpan MaxDuration(string name)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            TimeSpan maxDuration = TimeSpan.Zero;
            Parallel.ForEach(benchmarkDictionaries[name], (benchmarkKeyValuePair) =>
            {
                if (benchmarkKeyValuePair.Value.maxDuration > maxDuration)
                {
                    maxDuration = benchmarkKeyValuePair.Value.maxDuration;
                }
            });
            return maxDuration;
        }

        public static TimeSpan MinDuration(string name, int threadID)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            if (!benchmarkDictionaries[name].ContainsKey(threadID))
            {
                throw new Exception($"Benchmark '{name}' with threadID '{threadID}' does not exist!");
            }

            return benchmarkDictionaries[name][threadID].minDuration;
        }

        public static TimeSpan MinDuration(string name)
        {
            if (!benchmarkDictionaries.ContainsKey(name))
            {
                throw new Exception($"Benchmark '{name}' does not exist!");
            }

            TimeSpan minDuration = TimeSpan.MaxValue;
            Parallel.ForEach(benchmarkDictionaries[name], (benchmarkKeyValuePair) =>
            {
                if (benchmarkKeyValuePair.Value.minDuration < minDuration)
                {
                    minDuration = benchmarkKeyValuePair.Value.minDuration;
                }
            });
            return minDuration;
        }
        #endregion

        #region Methods
        private void Start()
        {
            if (running)
            {
                Stop();
            }
            
            running = true;
            stopwatch.Restart();
        }

        private void Stop()
        {
            if (!running)
            {
                Start();
            }

            running = false;
            stopwatch.Stop();
            samples++;
            latestDuration = stopwatch.Elapsed;
            latestDurationSerializable = new SerializableTimeSpan(latestDuration);
            durationSum += latestDuration;
            durationSumSerializable = new SerializableTimeSpan(durationSum);
            averageDuration = durationSum / samples;
            averageDurationSerializable = new SerializableTimeSpan(averageDuration);
            if (maxDuration < latestDuration)
            {
                maxDuration = latestDuration;
                maxDurationSerializable = new SerializableTimeSpan(maxDuration);
            }
            if (minDuration > latestDuration)
            {
                minDuration = latestDuration;
                minDurationSerializable = new SerializableTimeSpan(minDuration);
            }
        }
        #endregion
    }
}
