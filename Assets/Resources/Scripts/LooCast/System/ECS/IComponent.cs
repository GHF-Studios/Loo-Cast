using System;

namespace LooCast.System.ECS
{
    using LooCast.System.Lifecycle.Initialization;
    using LooCast.System.Lifecycle.Termination;
    using LooCast.System.Serialization;
    
    public interface IComponent : IPreInitializationPhase, IInitializationPhase, IPostInitializationPhase, IPreTerminationPhase, ITerminationPhase, IPostTerminationPhase
    {
        #region Interfaces
        public interface IMetaData : ISerializableObject
        {
            Guid ComponentID { get; set; }
            string AssemblyQualifiedComponentTypeName { get; set; }
            string AssemblyQualifiedComponentMetaDataTypeName { get; set; }
            string AssemblyQualifiedComponentDataTypeName { get; set; }
        }
        
        public interface IData : ISerializableObject
        {
            string AssemblyQualifiedComponentTypeName { get; set; }
            string AssemblyQualifiedComponentMetaDataTypeName { get; set; }
            string AssemblyQualifiedComponentDataTypeName { get; set; }
        }
        #endregion

        #region Properties
        Guid ComponentID { get; }
        IEntity Entity { get; }
        #endregion

        #region Methods
        /// <summary>
        /// Automatically called when this component is being created. 
        /// Do NOT manually call this method! 
        /// </summary>
        void OnCreate();
        
        /// <summary>
        /// Automatically called when this component is destroyed. 
        /// Do NOT manually call this method! 
        /// </summary>
        void OnDestroy();
        
        /// <summary>
        /// Automatically called when the component is internally being created.
        /// Do NOT manually call this method!
        /// </summary>
        void Create_INTERNAL(Type componentType, Type componentMetaDataType, Type componentDataType, IEntity entity);

        /// <summary>
        /// Automatically called when the component is internally being destroyed.
        /// Do NOT manually call this method!
        /// </summary>
        void Destroy_INTERNAL();

        #region Data Management
        IMetaData GetComponentMetaData();
        void SetComponentMetaData(IMetaData componentMetaData);

        IData GetComponentData();
        void SetComponentData(IData componentData);
        #endregion

        #endregion
    }
}
