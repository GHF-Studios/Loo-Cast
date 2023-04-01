using System;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;
    using global::LooCast.System.MetaData;

    public class GameObject : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => gameObjectIdentifier;
        public GameObjectIdentifier GameObjectIdentifier => gameObjectIdentifier;

        public Guid GameObjectInstanceGUID => gameObjectInstanceGUID;
        public UnityEngine.GameObject GameObjectInstance => gameObjectInstance;
        
        public Type ContainingType => containingType;
        public Type BehaviourType => behaviourType;
        public Type DataType => dataType;
        
#nullable enable 
        public GameObject? ParentGameObject => parentGameObject;
#nullable disable
        public GameObjectRegistry ChildGameObjects => childGameObjects;
        public ComponentRegistry ContainedComponents => containedComponents;
        #endregion

        #region Fields
#nullable enable 
        private GameObjectIdentifier? gameObjectIdentifier;
#nullable disable

        private Guid gameObjectInstanceGUID;
        private UnityEngine.GameObject gameObjectInstance;

        private Type containingType;
        private Type behaviourType;
        private Type dataType;

#nullable enable
        private GameObject? parentGameObject;
#nullable disable
        private GameObjectRegistry childGameObjects;
        private ComponentRegistry containedComponents;
        #endregion

        #region Constructors
        public GameObject(GameObjectMetaData gameObjectMetaData)
        {
            TypeManager typeManager = TypeManager.Instance;

            gameObjectInstanceGUID = Guid.NewGuid();
            gameObjectInstance = new UnityEngine.GameObject();
            
            containingType = typeManager.GetType(gameObjectMetaData.TypeIdentifier);
            behaviourType = typeManager.GetType(gameObjectMetaData.BehaviourTypeIdentifier);
            this.dataType = typeManager.GetType(gameObjectMetaData.DataTypeIdentifier);

            Type extendeMonoBehaviourType = typeManager.GetType("LooCast.System:ExtendedMonoBehaviour");
            Type dataType = typeManager.GetType("LooCast.System:Data");
            
            Type.CheckBaseType(behaviourType, extendeMonoBehaviourType);
            Type.CheckBaseType(this.dataType, dataType);

            global::System.Reflection.MethodInfo addComponentMethod = gameObjectInstance.GetType().GetMethod("AddComponent", global::System.Type.EmptyTypes);
            object componentTypeInstance = Activator.CreateInstance(behaviourType.CSSystemType);
            addComponentMethod.Invoke(gameObjectInstance, new[] { componentTypeInstance });

            parentGameObject = gameObjectMetaData.ParentGameObject;
            childGameObjects = new GameObjectRegistry();
            containedComponents = new ComponentRegistry();
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is GameObject otherGameObject)
            {
                return Equals(otherGameObject);
            }
            return false;
        }

        public bool Equals(GameObject otherGameObject)
        {
            return GameObjectIdentifier.Equals(otherGameObject.GameObjectIdentifier);
        }

        public override int GetHashCode()
        {
            return GameObjectIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return GameObjectIdentifier.ToString();
        }
        #endregion
    }
}
