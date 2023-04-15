namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class SystemObjectRegistry : Registry<SystemObjectIdentifier, SystemObject>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
