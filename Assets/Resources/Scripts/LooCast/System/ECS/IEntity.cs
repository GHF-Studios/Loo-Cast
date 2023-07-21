using System;

namespace LooCast.System.ECS
{
    using LooCast.System.Lifecycle.Setup;
    using LooCast.System.Lifecycle.Initialization;
    using LooCast.System.Lifecycle.Termination;

    public interface IEntity : IPreInitializationPhase, IInitializationPhase, IPostInitializationPhase, IPreTerminationPhase, ITerminationPhase, IPostTerminationPhase
    {
        #region Properties
        Guid EntityID { get; }
        UnityBridge UnityBridge { get; }
        bool IsUnityBridgeEnabled { get; }
        #endregion

        #region Methods
        void OnCreate();
        void OnDestroy();
        
        void EnableUnityBridge();
        void DisableUnityBridge();

        ComponentType AddComponent<ComponentType>() where ComponentType : IComponent, new();
        void RemoveComponent<ComponentType>() where ComponentType : IComponent, new();
        bool ContainsComponent<ComponentType>() where ComponentType : IComponent, new();
        ComponentType GetComponent<ComponentType>() where ComponentType : IComponent, new();
        bool TryGetComponent<ComponentType>(out IComponent component) where ComponentType : IComponent, new();
        #endregion
    }
}
