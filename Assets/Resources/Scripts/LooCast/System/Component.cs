using System;
using UnityEngine;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public class Component : MonoBehaviour, ILooCastObject
    {
        #region Properties
        public Identifier Identifier => componentIdentifier;
        public ComponentIdentifier ComponentIdentifier => componentIdentifier;
        public Guid ComponentInstanceGUID => componentInstanceGUID;
        public Type ComponentType => componentType;
        public GameObject ContainingGameObject => containingGameObject;
        #endregion

        #region Fields
        private ComponentIdentifier componentIdentifier;
        private Guid componentInstanceGUID;
        private Type componentType;
        private GameObject containingGameObject;
        #endregion

        #region Static Methods
        public static T Create<T>(GameObject containingGameObject) where T : Component
        {
            Component component = containingGameObject.UnityEngineGameObject.AddComponent<T>();
            component.componentInstanceGUID = Guid.NewGuid();
            component.componentType = new Type<T>();
            component.containingGameObject = containingGameObject;
            component.componentIdentifier = new ComponentIdentifier(containingGameObject.GameObjectIdentifier, component.componentType.TypeIdentifier, component.componentInstanceGUID);
            containingGameObject.ContainedComponents.Add(component);
            component.OnPreConstruct();
            component.OnConstruct();
            component.OnPostConstruct();
            return (T)component;
        }
        #endregion

        #region Methods
        protected virtual void OnPreConstruct()
        {

        }

        protected virtual void OnConstruct()
        {

        }

        protected virtual void OnPostConstruct()
        {

        }
        #endregion
    }
}
