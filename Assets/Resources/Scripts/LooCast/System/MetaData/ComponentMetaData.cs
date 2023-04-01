using System;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using global::LooCast.System;
    using global::LooCast.System.Identifiers;

    [Serializable]
    public class ComponentMetaData : IMetaData
    {
        #region Properties
        public TypeIdentifier TypeIdentifier => typeIdentifier;
        public TypeIdentifier BehaviourTypeIdentifier => behaviourTypeIdentifier;
        public TypeIdentifier DataTypeIdentifier => dataTypeIdentifier;
        public GameObject ContainingGameObject => containingGameObject;
#nullable enable
        public global::System.Collections.Generic.List<IMetaData>? Dependencies
        {
            get
            {
                return dependencies;
            }
            set
            {
                dependencies = value;
            }
        }
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier typeIdentifier;
        [SerializeField] private TypeIdentifier behaviourTypeIdentifier;
        [SerializeField] private TypeIdentifier dataTypeIdentifier;
        [SerializeField] private GameObject containingGameObject;
#nullable enable
        [SerializeField] private global::System.Collections.Generic.List<IMetaData>? dependencies;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public ComponentMetaData(TypeIdentifier typeIdentifier, TypeIdentifier behaviourTypeIdentifier, TypeIdentifier dataTypeIdentifier, GameObject containingGameObject, global::System.Collections.Generic.List<IMetaData>? dependencies = null)
        {
            this.typeIdentifier = typeIdentifier;
            this.behaviourTypeIdentifier = behaviourTypeIdentifier;
            this.dataTypeIdentifier = dataTypeIdentifier;
            this.containingGameObject = containingGameObject;
            this.dependencies = dependencies;
        }
#nullable disable
        #endregion

        #region Methods
        public virtual void Validate()
        {
            if (typeIdentifier == null)
            {
                throw new ArgumentNullException(nameof(typeIdentifier));
            }
            
            if (behaviourTypeIdentifier == null)
            {
                throw new ArgumentNullException(nameof(behaviourTypeIdentifier));
            }
            
            if (dataTypeIdentifier == null)
            {
                throw new ArgumentNullException(nameof(dataTypeIdentifier));
            }
            
            if (containingGameObject == null)
            {
                throw new ArgumentNullException(nameof(containingGameObject));
            }
        }
        #endregion
    }
}
