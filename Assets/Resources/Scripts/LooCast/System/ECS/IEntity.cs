using System;

namespace LooCast.System.ECS
{
    public interface IEntity
    {
        #region Properties
        Guid EntityID { get; }
        UnityBridge UnityBridge { get; }
        bool IsUnityBridgeEnabled { get; }
        #endregion

        #region Methods
        public void EnableUnityBridge();
        public void DisableUnityBridge();

        public ComponentType AddComponent<ComponentType>() where ComponentType : IComponent, new();
        public void RemoveComponent<ComponentType>() where ComponentType : IComponent, new();
        public bool ContainsComponent<ComponentType>() where ComponentType : IComponent, new();
        public ComponentType GetComponent<ComponentType>() where ComponentType : IComponent, new();
        public bool TryGetComponent<ComponentType>(out IComponent component) where ComponentType : IComponent, new();
        #endregion
    }
}
