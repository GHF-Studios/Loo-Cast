using UnityEngine;

namespace LooCast.Core.Manager
{
    public abstract class Manager : MonoBehaviour
    {
        public abstract void PreInitialize();
        public abstract void Initialize();
        public abstract void PostInitialize();
    }
}