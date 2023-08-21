namespace LooCast
{
    public interface IEntrypoint
    {
        void PreInitialize();
        void Initialize();
        void PostInitialize();
    }
}
