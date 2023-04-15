namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class TypeRegistry : Registry<TypeIdentifier, Type>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
