namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class ComponentRegistry : Registry<ComponentIdentifier, Component>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
