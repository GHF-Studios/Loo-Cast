using System;

namespace LooCast.System.ECS
{
    using LooCast.System.Serialization;
    using LooCast.System.Lifecycle.Initialization;
    using LooCast.System.Lifecycle.Termination;
    
    public interface IComponent : ISerializable, IPreInitializationPhase, IInitializationPhase, IPostInitializationPhase, IPreTerminationPhase, ITerminationPhase, IPostTerminationPhase
    {
        #region Interfaces
        public interface IMetaData : Serialization.IMetaData
        {
            string AssemblyQualifiedComponentTypeName { get; set; }
            string AssemblyQualifiedComponentMetaDataTypeName { get; set; }
            string AssemblyQualifiedComponentDataTypeName { get; set; }
        }
        
        public interface IData : Serialization.IData
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
        #endregion
    }
}
