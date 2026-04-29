using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Station
{
    using LooCast.Core;
    using LooCast.System;
    using Data;
    using Data.Runtime;

    [DisallowMultipleComponent]
    public abstract class Station : ExtendedMonoBehaviour
    {
        public StationRuntimeSet RuntimeSet;

        protected void Initialize(StationData data)
        {
            RuntimeSet.Add(this);
        }

        public void Kill()
        {
            RuntimeSet.Remove(this);
            Destroy(gameObject);
        }
    } 
}
