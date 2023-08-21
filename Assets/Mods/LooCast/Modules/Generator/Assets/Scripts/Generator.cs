using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using Core;

    public abstract class Generator : Component
    {
        public abstract void Initialize();

        public abstract void Generate();
    } 
}