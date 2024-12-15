using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using LooCast.Core;
    using LooCast.System;

    public abstract class Generator : ExtendedMonoBehaviour
    {
        public abstract void Initialize();

        public abstract void Generate();
    } 
}
