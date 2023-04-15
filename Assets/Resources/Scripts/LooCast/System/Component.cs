using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;

    public abstract class Component : UnityEngine.MonoBehaviour, ILooCastObject
    {
        #region Properties
        public Identifier Identifier => componentMetaData.ComponentIdentifier;
        public ComponentMetaData ComponentMetaData => componentMetaData;
        public UnityEngine.Component UnityEngineComponent => unityEngineComponent;
        public GameObject ContainingGameObject => componentMetaData.ContainingGameObject;
        #endregion

        #region Fields
        private ComponentMetaData componentMetaData;
        private UnityEngine.Component unityEngineComponent;
        #endregion

        #region Static Methods
#nullable enable
        public static ComponentType CreateComponent<ComponentType, ComponentMetaDataType>(GameObject containingGameObject)
            where ComponentType : Component, new()
            where ComponentMetaDataType : ComponentMetaData, new()
        {
            ComponentType component = containingGameObject.UnityEngineGameObject.AddComponent<ComponentType>();
            ComponentMetaDataType componentMetaData = Activator.CreateInstance<ComponentMetaDataType>();
            component.CreateMetaData<ComponentType, ComponentMetaDataType>(containingGameObject, ref componentMetaData);
            component.SetMetaData(componentMetaData);
            component.PreConstruct();
            component.Construct();
            component.PostConstruct();
            return component;
        }

        public static ComponentType CreateComponent<ComponentType>(GameObject containingGameObject)
            where ComponentType : Component, new()
        {
            ComponentType component = containingGameObject.UnityEngineGameObject.AddComponent<ComponentType>();
            ComponentMetaData componentMetaData = new ComponentMetaData();
            component.CreateMetaData<ComponentType, ComponentMetaData>(containingGameObject, ref componentMetaData);
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
            ComponentType component = componentMetaData.ContainingGameObject.UnityEngineGameObject.AddComponent<ComponentType>();
            component.SetMetaData(componentMetaData);
            component.PreConstruct();
            component.Construct();
            component.PostConstruct();
            return component;
        }
#nullable disable
        #endregion

        #region Methods
        protected virtual void CreateMetaData<ComponentType, ComponentMetaDataType>(GameObject containingGameObject, ref ComponentMetaDataType componentMetaData)
            where ComponentType : Component, new()
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
}
