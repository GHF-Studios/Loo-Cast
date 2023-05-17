namespace LooCast.Core.Registries
{
    using global::LooCast.Core.Identifiers;

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
