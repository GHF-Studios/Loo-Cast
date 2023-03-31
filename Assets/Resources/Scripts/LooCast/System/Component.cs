using System;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Managers;
    using LooCast.System.Registries;
    using Unity.VisualScripting;

    public class Component : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => componentIdentifier;
        public ComponentIdentifier ComponentIdentifier => componentIdentifier;

        public Guid ComponentInstanceGUID => componentInstanceGUID;
        public UnityEngine.Component ComponentInstance => componentInstance;

        public Type ContainingType => containingType;
        public GameObject ContainingGameObject => containingGameObject;
        #endregion

        #region Fields
#nullable enable 
        private ComponentIdentifier? componentIdentifier;
#nullable disable

        private Guid componentInstanceGUID;
        private UnityEngine.Component componentInstance;

        private Type containingType;
        private GameObject containingGameObject;
        #endregion

        #region Constructors
        public Component(TypeIdentifier typeIdentifier, GameObject containingGameObject)
        {
            TypeManager typeManager = TypeManager.Instance;

            componentIdentifier = new ComponentIdentifier(containingGameObject.GameObjectIdentifier, typeIdentifier, Guid.NewGuid());
            componentInstanceGUID = componentIdentifier.ComponentInstanceGUID;
            containingType = typeManager.GetType(typeIdentifier);
            this.containingGameObject = containingGameObject;

            global::System.Reflection.MethodInfo addComponentMethod = containingGameObject.GameObjectInstance.GetType().GetMethod("AddComponent", global::System.Type.EmptyTypes);
            global::System.Type genericTypeArgument = containingType.CSSystemType;
            object componentTypeInstance = Activator.CreateInstance(genericTypeArgument);
            componentInstance = (UnityEngine.Component)addComponentMethod.Invoke(containingGameObject.GameObjectInstance, new[] { componentTypeInstance });
            containingGameObject.ContainedComponents.Add(ComponentIdentifier, this);
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is Component otherComponent)
            {
                return Equals(otherComponent);
            }
            return false;
        }

        public bool Equals(Component otherComponent)
        {
            return ComponentIdentifier.Equals(otherComponent.ComponentIdentifier);
        }

        public override int GetHashCode()
        {
            return ComponentIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return ComponentIdentifier.ToString();
        }
        #endregion
    }
}
