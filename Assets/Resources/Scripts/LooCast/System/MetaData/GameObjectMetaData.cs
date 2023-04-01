using System;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using global::LooCast.System;
    using global::LooCast.System.Identifiers;
    
    [Serializable]
    public class GameObjectMetaData : IMetaData
    {
        #region Properties
        public TypeIdentifier TypeIdentifier => typeIdentifier;
        public TypeIdentifier BehaviourTypeIdentifier => behaviourTypeIdentifier;
        public TypeIdentifier DataTypeIdentifier => dataTypeIdentifier;
#nullable enable
        public GameObject? ParentGameObject => parentGameObject;
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
#nullable enable
        [SerializeField] private GameObject? parentGameObject;
        [SerializeField] private global::System.Collections.Generic.List<IMetaData>? dependencies;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public GameObjectMetaData(TypeIdentifier typeIdentifier, TypeIdentifier behaviourTypeIdentifier, TypeIdentifier dataTypeIdentifier, GameObject? parentGameObject = null, global::System.Collections.Generic.List<IMetaData>? dependencies = null)
        {
            this.typeIdentifier = typeIdentifier;
            this.behaviourTypeIdentifier = behaviourTypeIdentifier;
            this.dataTypeIdentifier = dataTypeIdentifier;
            this.parentGameObject = parentGameObject;
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
        }
        #endregion
    }
}
