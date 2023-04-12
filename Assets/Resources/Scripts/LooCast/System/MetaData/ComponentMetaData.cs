using System;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;

    [Serializable]
    public class ComponentMetaData
    {
        #region Properties
        public ComponentIdentifier ComponentIdentifier
        {
            get
            {
                return componentIdentifier;
            }
            set
            {
                componentIdentifier = value;
            }
        }
        public Guid ComponentInstanceGUID
        {
            get
            {
                return componentIdentifier.ComponentInstanceGUID;
            }
        }
        public Type ComponentType
        {
            get
            {
                return TypeManager.Instance.GetType(componentIdentifier.ComponentTypeIdentifier);
            }
        }
        public GameObject ContainingGameObject
        {
            get
            {
                if (containingGameObject == null)
                {
                    if (containingGameObjectIdentifier != null)
                    {
                        containingGameObject = GameObjectManager.Instance.GetGameObject(containingGameObjectIdentifier);
                    }
                }
                return containingGameObject;
            }
            set
            {
                containingGameObject = value;
                containingGameObjectIdentifier = containingGameObject == null ? null : containingGameObject.GameObjectMetaData.GameObjectIdentifier;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private ComponentIdentifier componentIdentifier;
        [SerializeField] private GameObjectIdentifier containingGameObjectIdentifier;

        private GameObject containingGameObject;
        #endregion

        #region Methods
        public virtual bool Validate()
        {
            if (componentIdentifier == null)
            {
                return false;
            }
            if (containingGameObjectIdentifier == null)
            {
                return false;
            }
            return true;
        }
        #endregion
    }
}
