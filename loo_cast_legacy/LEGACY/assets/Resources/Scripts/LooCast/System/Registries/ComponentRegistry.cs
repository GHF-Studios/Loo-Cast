namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class ComponentRegistry : Registry<ComponentIdentifier, Component>
    {
        public ComponentRegistry() : base("LooCast.System.Registries.ComponentRegistry")
        {
            
        }
    }
}
