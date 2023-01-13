using System;
using UnityEngine;

namespace LooCast.Data
{
    public abstract class DynamicData : ScriptableObject
    {
        public abstract void Save();

        public abstract void Load();

        public abstract void LoadDefault();
    }
}
