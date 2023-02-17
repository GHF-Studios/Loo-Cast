namespace LooCast.System
{
    using System.Identification;

    public interface IGameObject : IObject, IGameObjectIdentifiable
    {
        #region Properties
        IUnityInstance GameObjectInstance { get; }
        #endregion
    }
}