namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;
    using LooCast.System.Types;

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
