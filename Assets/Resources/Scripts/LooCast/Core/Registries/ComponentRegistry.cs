namespace LooCast.Core.Registries
{
    using global::LooCast.Core.Identifiers;
    using global::LooCast.Core.Types;

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
