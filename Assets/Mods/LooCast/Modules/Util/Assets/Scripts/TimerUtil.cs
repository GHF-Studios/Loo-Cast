using UnityEngine;
using System;
using System.Collections.Generic;

namespace LooCast.Util
{
    public class TimerUtil : MonoBehaviour
    {
        #region Classes
        public class Timer
        {
            #region Delegates
            public delegate void UpdateDelegate(float deltaTime);
            #endregion

            #region Properties
            public UpdateDelegate UpdateAction
            {
                get
                {
                    return (deltaTime) =>
                    {
                        Update(deltaTime);
                    };
                }
            }
            public int ID { get; private set; }
            public TimeSpan Duration { get; private set; }
            public bool AutoReset { get; private set; }
            public bool Activated { get; private set; }
            public Action ElapsedAction { get; private set; }
            public DateTime ActivationTime { get; private set; }
            public DateTime CurrentTime { get; private set; }
            #endregion

            #region Constructors
            public Timer(int id, float duration, bool autoActivated, bool autoReset)
            {
                ID = id;
                Duration = TimeSpan.FromSeconds(duration);
                AutoReset = autoReset;

                if (autoActivated)
                {
                    Start();
                }
            }
            #endregion

            #region Methods
            public void Start()
            {
                ActivationTime = DateTime.Now;
                CurrentTime = DateTime.Now;
                Activated = true;
            }

            public void Stop()
            {
                Activated = false;
            }

            public void AddElapsedAction(Action elapsedAction)
            {
                ElapsedAction += elapsedAction;
            }

            public void RemoveElapsedAction(Action elapsedAction)
            {
                ElapsedAction -= elapsedAction;
            }

            private void Update(float deltaTime)
            {
                if (Activated)
                {
                    CurrentTime += TimeSpan.FromSeconds(deltaTime);
                    TimeSpan activeTimeSpan = new TimeSpan(CurrentTime.Ticks - ActivationTime.Ticks);
                    if (activeTimeSpan >= Duration)
                    {
                        Elapse();
                    }
                }
            }

            private void Elapse()
            {
                Activated = false;
                ElapsedAction();
                if (AutoReset)
                {
                    Start();
                }
            }
            #endregion
        }
        #endregion

        #region Static Fields
        private static TimerUtil instance;
        private static int idCounter = 0;
        #endregion

        #region Static Methods
        public static Timer CreateTimer(float duration, bool autoActivated = false, bool autoReset = false)
        {
            if (instance == null)
            {
                InitializeInstance();
            }

            Timer timer = new Timer(idCounter, duration, autoActivated, autoReset);
            instance.timers.Add(timer);
            idCounter++;
            return timer;
        }

        public static void DeleteTimer(Timer timer)
        {
            if (instance == null)
            {
                InitializeInstance();
            }

            instance.timers.Remove(timer);
        }

        public static void InitializeInstance()
        {
            if (instance != null)
            {
                throw new Exception("Cannot have multiple instances of TimerUtil!");
            }
            GameObject instanceObject = new GameObject("[TimerUtil]");
            instanceObject.layer = 31;
            instanceObject.tag = "INTERNAL";
            instance = instanceObject.AddComponent<TimerUtil>();
            DontDestroyOnLoad(instance);
            Debug.Log("[TimerUtil] Initialized.");
        }
        #endregion

        #region Fields
        private List<Timer> timers = new List<Timer>();
        #endregion

        #region Unity Callbacks
        private void Update()
        {
            foreach (Timer timer in timers)
            {
                timer.UpdateAction(Time.deltaTime);
            }
        }
        #endregion
    }
}