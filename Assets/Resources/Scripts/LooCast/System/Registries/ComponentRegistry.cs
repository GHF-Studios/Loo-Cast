namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Types;

    public sealed class ComponentRegistry : Registry<IComponentIdentifier, IComponentType.IComponent>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
