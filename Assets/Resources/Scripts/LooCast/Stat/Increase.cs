﻿using System.Timers;
using UnityEngine.Events;

namespace LooCast.Stat
{
    public class Increase
    {
        public int Value { get; private set; }

        public Increase(int increase)
        {
            Value = increase;
        }
    }
}
