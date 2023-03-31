namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class NamespaceRegistry : Registry<NamespaceIdentifier, Namespace>
    {
        public NamespaceRegistry() : base("LooCast.System.Registries.NamespaceRegistry")
        {
            
        }
    }
}
