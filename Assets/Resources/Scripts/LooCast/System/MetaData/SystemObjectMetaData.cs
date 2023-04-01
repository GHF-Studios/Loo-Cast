using System;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using global::LooCast.System;
    using global::LooCast.System.Identifiers;

    [Serializable]
    public class SystemObjectMetaData : IMetaData
    {
        #region Properties
        public TypeIdentifier TypeIdentifier => typeIdentifier;
#nullable enable
        public SystemObject? ParentSystemObject => parentSystemObject;
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
        [SerializeField] private SystemObject? parentSystemObject;
        [SerializeField] private global::System.Collections.Generic.List<IMetaData>? dependencies;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public SystemObjectMetaData(TypeIdentifier typeIdentifier, SystemObject? parentSystemObject = null, global::System.Collections.Generic.List<IMetaData>? dependencies = null)
        {
            this.typeIdentifier = typeIdentifier;
            this.parentSystemObject = parentSystemObject;
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
