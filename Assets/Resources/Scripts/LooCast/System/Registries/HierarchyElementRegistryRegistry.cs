namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class HierarchyElementRegistryRegistry : Registry<TypeIdentifier, HierarchyElementRegistry>
    {
        public HierarchyElementRegistryRegistry() : base("LooCast.System.Registries:HierarchyElementRegistryRegistry")
        {
            
        }
    }
}
