namespace LooCast.Core.Registries
{
    using LooCast.Core.Identifiers;
    using LooCast.Core.Types;

    public sealed class GameObjectRegistry : Registry<IGameObjectIdentifier, IGameObjectType.IGameObject>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
