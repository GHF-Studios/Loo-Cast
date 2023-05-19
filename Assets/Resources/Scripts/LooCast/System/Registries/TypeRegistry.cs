namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;

    public sealed class TypeRegistry : Registry<TypeIdentifier, IType>
    {
        #region Constructors
        public TypeRegistry() : base(MainManager.Instance.MainRegistry)
        {

        }
        #endregion
    }
}
