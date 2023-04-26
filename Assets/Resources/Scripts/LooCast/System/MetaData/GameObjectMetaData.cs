using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Identifiers;

    [Serializable]
    public class GameObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public GameObjectIdentifier GameObjectIdentifier
        {
            get
            {
                return gameObjectIdentifier;
            }
            set
            {
                gameObjectIdentifier = value;
            }
        }
        public Guid GameObjectInstanceGUID
        {
            get
            {
                return gameObjectIdentifier.GameObjectInstanceGUID;
            }
        }
        public Type GameObjectType
        {
            get
            {
                return TypeManager.Instance.GetType(gameObjectIdentifier.GameObjectTypeIdentifier);
            }
        }
#nullable enable
        public GameObject? ParentGameObject
        {
            get
            {
                if (parentGameObject == null)
                {
                    if (parentGameObjectIdentifier != null)
                    {
                        parentGameObject = GameObjectManager.Instance.GetGameObject(parentGameObjectIdentifier);
                    }
                }
                return parentGameObject;
            }
            set
            {
                parentGameObject = value;
                parentGameObjectIdentifier = parentGameObject == null ? null : parentGameObject.GameObjectMetaData.GameObjectIdentifier;
            }
        }
#nullable disable
        public List<GameObject> ChildGameObjects
        {
            get
            {
                if (childGameObjects == null)
                {
                    childGameObjects = new List<GameObject>();
                    if (childGameObjectIdentifiers.Count > 0)
                    {
                        foreach (GameObjectIdentifier childGameObjectIdentifier in childGameObjectIdentifiers)
                        {
                            GameObject childGameObject = GameObjectManager.Instance.GetGameObject(childGameObjectIdentifier);
                            childGameObjects.Add(childGameObject);
                        }
                    }
                }
                return childGameObjects;
            }
            set
            {
                childGameObjects = value;
                childGameObjectIdentifiers = new SerializableList<GameObjectIdentifier>();
                if (childGameObjects != null && childGameObjects.Count > 0)
                {
                    for (int i = 0; i < childGameObjects.Count; i++)
                    {
                        childGameObjectIdentifiers.Add(childGameObjects[i].GameObjectMetaData.GameObjectIdentifier);
                    }
                }
            }
        }
        public List<Component> ContainedComponents
        {
            get
            {
                if (containedComponents == null)
                {
                    containedComponents = new List<Component>();
                    if (containedComponentIdentifiers.Count > 0)
                    {
                        foreach (ComponentIdentifier containedComponentIdentifier in containedComponentIdentifiers)
                        {
                            Component containedComponent = ComponentManager.Instance.GetComponent(containedComponentIdentifier);
                            containedComponents.Add(containedComponent);
                        }
                    }
                }
                return containedComponents;
            }
            set
            {
                containedComponents = value;
                containedComponentIdentifiers = new SerializableList<ComponentIdentifier>();
                if (containedComponents != null && containedComponents.Count > 0)
                {
                    for (int i = 0; i < containedComponents.Count; i++)
                    {
                        containedComponentIdentifiers.Add(containedComponents[i].ComponentMetaData.ComponentIdentifier);
                    }
                }
            }
        }
        #endregion

        #region Fields
        [SerializeField] private GameObjectIdentifier gameObjectIdentifier;
#nullable enable
        [SerializeField] private GameObjectIdentifier? parentGameObjectIdentifier;
#nullable disable
        [SerializeField] private SerializableList<GameObjectIdentifier> childGameObjectIdentifiers;
        [SerializeField] private SerializableList<ComponentIdentifier> containedComponentIdentifiers;

#nullable enable
        private GameObject? parentGameObject;
        private List<GameObject>? childGameObjects;
        private List<Component>? containedComponents;
#nullable disable
        #endregion

        #region Methods
        public virtual bool Validate()
        {
            if (gameObjectIdentifier == null)
            {
                return false;
            }
            if (childGameObjectIdentifiers == null)
            {
                return false;
            }
            if (containedComponentIdentifiers == null)
            {
                return false;
            }
            return true;
        }
        #endregion
    }
}
