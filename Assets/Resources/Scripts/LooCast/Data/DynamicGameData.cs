using System;
using UnityEngine;

namespace LooCast.Data
{
    public abstract class DynamicGameData : ScriptableObject
    {
        public abstract void Save();

        public abstract void Load();

        public abstract void LoadDefault();
    }
}
