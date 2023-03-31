using System;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;

    public class SystemObject : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => systemObjectIdentifier;
        public SystemObjectIdentifier SystemObjectIdentifier => systemObjectIdentifier;

        public Guid SystemObjectInstanceGUID => systemObjectInstanceGUID;
        public object SystemObjectInstance => systemObjectInstance;

        public Type ContainingType => containingType;

#nullable enable
        public SystemObject? ParentSystemObject => parentSystemObject;
#nullable disable
        public SystemObjectRegistry ChildSystemObjects => childSystemObjects;
        #endregion

        #region Fields
#nullable enable
        private SystemObjectIdentifier? systemObjectIdentifier;
#nullable disable

        private Guid systemObjectInstanceGUID;
        private object systemObjectInstance;

        private Type containingType;
        private Type behaviourType;
        private Type dataType;

#nullable enable
        private SystemObject? parentSystemObject;
#nullable disable
        private SystemObjectRegistry childSystemObjects;
        #endregion

        #region Constructors
        public SystemObject(TypeIdentifier typeIdentifier, TypeIdentifier behaviourTypeIdentifier, TypeIdentifier dataTypeIdentifier, SystemObject parentSystemObject = null)
        {
            TypeManager typeManager = TypeManager.Instance;

            systemObjectInstanceGUID = Guid.NewGuid();
            systemObjectInstance = new object();

            containingType = typeManager.GetType(typeIdentifier);
            behaviourType = typeManager.GetType(behaviourTypeIdentifier);
            this.dataType = typeManager.GetType(dataTypeIdentifier);

            Type extendeMonoBehaviourType = typeManager.GetType("LooCast.System:ExtendedMonoBehaviour");
            Type dataType = typeManager.GetType("LooCast.System:Data");

            Type.CheckBaseType(behaviourType, extendeMonoBehaviourType);
            Type.CheckBaseType(this.dataType, dataType);

            this.parentSystemObject = parentSystemObject;
            childSystemObjects = new SystemObjectRegistry();
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is SystemObject otherSystemObject)
            {
                return Equals(otherSystemObject);
            }
            return false;
        }

        public bool Equals(SystemObject otherSystemObject)
        {
            return SystemObjectIdentifier.Equals(otherSystemObject.SystemObjectIdentifier);
        }

        public override int GetHashCode()
        {
            return SystemObjectIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return SystemObjectIdentifier.ToString();
        }
        #endregion
    }
}
