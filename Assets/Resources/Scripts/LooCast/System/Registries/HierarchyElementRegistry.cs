namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class HierarchyElementRegistry : Registry<Identifier, IHierarchyElement>
    {
        public HierarchyElementRegistry() : base("LooCast.System.Registries:HierarchyElementRegistry")
        {
            
        }
    }
}
