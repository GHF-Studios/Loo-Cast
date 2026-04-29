namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class TypeRegistry : Registry<TypeIdentifier, Type>
    {
        public TypeRegistry() : base("LooCast.System.Registries.TypeRegistry")
        {
        }
    }
}
