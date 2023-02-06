using LooCast.Data;
using System;
using UnityEngine;

namespace LooCast.Observer.Data
{
    [CreateAssetMenu(fileName = "UniverseObserverData", menuName = "Data/Observer/UniverseObserverData", order = 0)]
    public sealed class UniverseObserverData : ScriptableObject
    {
        public IntDataReference ChunkLoadRadius;
    }
}
