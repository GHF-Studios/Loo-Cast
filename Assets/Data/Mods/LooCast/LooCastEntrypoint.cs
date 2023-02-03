using System;

namespace LooCast
{
    public class LooCastEntrypoint : Entrypoint
    {
        public abstract void PreInitialize();

        public abstract void Initialize();

        public abstract void PostInitialize();
    }
}
