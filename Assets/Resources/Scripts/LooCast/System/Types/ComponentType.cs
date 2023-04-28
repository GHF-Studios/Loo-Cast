using UnityEngine;
using System.Collections;
using System.Collections.Generic;
using System;

namespace LooCast.System.Types
{
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;
    using LooCast.System.Registries;

    public abstract class ComponentType<TInstance> : InstanceType<TInstance>, IComponentType
        where TInstance : ComponentType<TInstance>.Component, new()
    {
        #region Classes
        public abstract class Component : MonoBehaviour, IComponentType.IComponent
        {
            #region Properties
            public abstract IMetaData MetaData { get; set; }
            public abstract IInstanceMetaData InstanceMetaData { get; set; }
            public abstract IComponentMetaData ComponentMetaData { get; set; }

            public abstract IData Data { get; set; }
            public abstract IInstanceData InstanceData { get; set; }
            public abstract IComponentData ComponentData { get; set; }
            #endregion
            
            #region Static Methods
#nullable enable
            public static ComponentType CreateComponent<ComponentType, ComponentMetaDataType>(IGameObjectType.IGameObject containingGameObject)
                where ComponentType : Component, new()
                where ComponentMetaDataType : ComponentMetaData, new()
            {
                ComponentType component = containingGameObject.GameObjectMetaData.UnityEngineGameObject.AddComponent<ComponentType>();
                
                ComponentMetaDataType componentMetaData = Activator.CreateInstance<ComponentMetaDataType>();
                component.CreateMetaData<ComponentType, ComponentMetaDataType>(containingGameObject, ref componentMetaData);
                
                component.SetMetaData(componentMetaData);
                
                component.PreConstruct();
                component.Construct();
                component.PostConstruct();
                
                return component;
            }

            public static ComponentType CreateComponent<ComponentType, ComponentMetaDataType>(ComponentMetaDataType componentMetaData)
                where ComponentType : Component, new()
                where ComponentMetaDataType : ComponentMetaData, new()
            {
                ComponentType component = componentMetaData.ParentGameObject.GameObjectMetaData.UnityEngineGameObject.AddComponent<ComponentType>();
                
                component.SetMetaData(componentMetaData);
                
                component.PreConstruct();
                component.Construct();
                component.PostConstruct();
                
                return component;
            }
#nullable disable
            #endregion

            #region Methods
            public abstract bool Validate();

            protected virtual void CreateMetaData<ComponentType, ComponentMetaDataType>(IGameObjectType.IGameObject containingGameObject, ref ComponentMetaDataType componentMetaData)
                where ComponentType : UnityEngine.Component, new()
                where ComponentMetaDataType : ComponentMetaData, new()
            {
                componentMetaData.ComponentIdentifier = new ComponentIdentifier(containingGameObject.GameObjectMetaData.GameObjectIdentifier, TypeManager.Instance.GetType<ComponentType>().TypeIdentifier, Guid.NewGuid());
                componentMetaData.ContainingGameObject = containingGameObject;
            }

            public virtual void SetMetaData(ComponentMetaData componentMetaData)
            {
                this.componentMetaData = componentMetaData;
            }

            protected virtual void PreConstruct()
            {
                ContainingGameObject.ContainedComponents.Add(this);
            }

            protected virtual void Construct()
            {

            }

            protected virtual void PostConstruct()
            {

            }
            #endregion
        }
        #endregion

        #region Properties
        public abstract IComponentTypeMetaData ComponentTypeMetaData { get; set; }

        public abstract IComponentTypeData ComponentTypeData { get; set; }
        #endregion
    }
}
