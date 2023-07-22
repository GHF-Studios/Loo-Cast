using System;

namespace LooCast.System.ECS
{
    using LooCast.System.Serialization;
    using LooCast.System.Lifecycle.Initialization;
    using LooCast.System.Lifecycle.Termination;

    public interface IEntity : ISerializable, IPreInitializationPhase, IInitializationPhase, IPostInitializationPhase, IPreTerminationPhase, ITerminationPhase, IPostTerminationPhase
    {
        #region Interfaces
        public interface IMetaData : Serialization.IMetaData
        {
            string AssemblyQualifiedEntityTypeName { get; set; }
            string AssemblyQualifiedEntityMetaDataTypeName { get; set; }
            string AssemblyQualifiedEntityDataTypeName { get; set; }
            IComponent.IMetaData[] ComponentMetaDatas { get; set; }
        }

        public interface IData : Serialization.IData
        {
            string AssemblyQualifiedEntityTypeName { get; set; }
            string AssemblyQualifiedEntityMetaDataTypeName { get; set; }
            string AssemblyQualifiedEntityDataTypeName { get; set; }
            IComponent.IData[] ComponentDatas { get; set; }
        }
        #endregion
        
        #region Properties
        Guid EntityID { get; }
        UnityBridge UnityBridge { get; }
        bool IsUnityBridgeEnabled { get; }
        #endregion

        #region Methods
        /// <summary>
        /// Automatically called when this entity is being created. 
        /// Do NOT manually call this method! 
        /// </summary>
        void OnCreate();
        
        /// <summary>
        /// Automatically called when this entity is being destroyed. 
        /// Do NOT manually call this method! 
        /// </summary>
        void OnDestroy();
        
        /// <summary>
        /// Automatically called when the entity is internally being created.
        /// Do NOT manually call this method!
        /// </summary>
        void Create_INTERNAL(Type entityType, Type entityMetaDataType, Type entityDataType);

        /// <summary>
        /// Automatically called when the entity is internally being destroyed.
        /// Do NOT manually call this method!
        /// </summary>
        void Destroy_INTERNAL();
        
        void EnableUnityBridge();
        void DisableUnityBridge();

        ComponentType AddComponent<ComponentType, ComponentMetaDataType, ComponentDataType>()
            where ComponentType : IComponent, new()
            where ComponentMetaDataType : IComponent.IMetaData, new()
            where ComponentDataType : IComponent.IData, new();
        IComponent AddComponent(Type newComponentType, Type newComponentMetaDataType, Type newComponentDataType);
        void RemoveComponent<ComponentType>() where ComponentType : IComponent, new();
        bool ContainsComponent<ComponentType>() where ComponentType : IComponent, new();
        ComponentType GetComponent<ComponentType>() where ComponentType : IComponent, new();
        bool TryGetComponent<ComponentType>(out IComponent component) where ComponentType : IComponent, new();
        #endregion
    }
}
