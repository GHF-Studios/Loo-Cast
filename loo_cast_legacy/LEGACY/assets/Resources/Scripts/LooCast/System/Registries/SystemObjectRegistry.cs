namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class SystemObjectRegistry : Registry<SystemObjectIdentifier, SystemObject>
    {
        public SystemObjectRegistry() : base("LooCast.System.Registries.SystemObjectRegistry")
        {
        }
    }
}
