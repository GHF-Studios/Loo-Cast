using System;

namespace LooCast
{
    public abstract class Entrypoint
    {
        public abstract void PreInitialize();

        public abstract void Initialize();

        public abstract void PostInitialize();
    }
}
