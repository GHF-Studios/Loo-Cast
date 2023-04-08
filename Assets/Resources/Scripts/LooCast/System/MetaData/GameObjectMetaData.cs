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
#nullable enable
        [SerializeField] private GameObject? parentGameObject;
        [SerializeField] private global::System.Collections.Generic.List<IMetaData>? dependencies;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public GameObjectMetaData(TypeIdentifier typeIdentifier, GameObject? parentGameObject = null, global::System.Collections.Generic.List<IMetaData>? dependencies = null)
        {
            this.typeIdentifier = typeIdentifier;
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
        }
        #endregion
    }
}
