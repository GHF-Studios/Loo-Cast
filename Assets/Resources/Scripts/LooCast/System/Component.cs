using System;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;

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
        private Type behaviourType;
        private Type dataType;
        
        private GameObject containingGameObject;
        #endregion

        #region Constructors
        public Component(TypeIdentifier typeIdentifier, TypeIdentifier behaviourTypeIdentifier, TypeIdentifier dataTypeIdentifier, GameObject containingGameObject)
        {
            TypeManager typeManager = TypeManager.Instance;

            componentIdentifier = new ComponentIdentifier(containingGameObject.GameObjectIdentifier, typeIdentifier, Guid.NewGuid());
            componentInstanceGUID = componentIdentifier.ComponentInstanceGUID;
            
            containingType = typeManager.GetType(typeIdentifier);
            behaviourType = typeManager.GetType(behaviourTypeIdentifier);
            this.dataType = typeManager.GetType(dataTypeIdentifier);

            Type extendeMonoBehaviourType = typeManager.GetType("LooCast.System:ExtendedMonoBehaviour");
            Type dataType = typeManager.GetType("LooCast.System:Data");

            Type.CheckBaseType(behaviourType, extendeMonoBehaviourType);
            Type.CheckBaseType(this.dataType, dataType);

            global::System.Reflection.MethodInfo addComponentMethod = containingGameObject.GameObjectInstance.GetType().GetMethod("AddComponent", global::System.Type.EmptyTypes);
            object componentTypeInstance = Activator.CreateInstance(containingType.CSSystemType);
            componentInstance = (UnityEngine.Component)addComponentMethod.Invoke(containingGameObject.GameObjectInstance, new[] { componentTypeInstance });

            this.containingGameObject = containingGameObject;
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
