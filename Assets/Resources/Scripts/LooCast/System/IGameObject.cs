using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IGameObject : IUnityInstance, IGameObjectIdentifiable
    {
        #region Properties
        public IGameObjectType GameObjectType { get; }
        public IGameObject ParentGameObject { get; }
        public List<IGameObject> ChildGameObjects { get; }
        #endregion
    }
}
