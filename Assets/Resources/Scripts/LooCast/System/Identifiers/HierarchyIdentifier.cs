using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public class HierarchyIdentifier : Identifier, IHierarchyIdentifier
    {
        #region Properties
        public ITypeIdentifier HierarchyTypeIdentifier => hierarchyTypeIdentifier;
        #endregion

        #region Fields
        [SerializeField] private readonly ITypeIdentifier hierarchyTypeIdentifier;
        #endregion

        #region Constructors
        protected HierarchyIdentifier(ITypeIdentifier hierarchyTypeIdentifier) : base(hierarchyTypeIdentifier.GUSID)
        {
            this.hierarchyTypeIdentifier = hierarchyTypeIdentifier;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static HierarchyIdentifier Parse(string hierarchyGUSID)
        {
            if (!TryParse(hierarchyGUSID, out HierarchyIdentifier? hierarchyIdentifier))
            {
                throw new ArgumentException($"'{hierarchyGUSID}' is not a valid hierarchy GUSID!");
            }

            return hierarchyIdentifier!;
        }

        public static HierarchyIdentifier Parse<HierarchyType>()
        where HierarchyType : IHierarchy
        {
            return Parse(typeof(HierarchyType));
        }

        public static HierarchyIdentifier Parse(Type hierarchyType)
        {
            return Parse(TypeIdentifier.Parse(hierarchyType));
        }

        public static bool TryParse(string hierarchyGUSID, out HierarchyIdentifier? hierarchyIdentifier)
        {
            hierarchyIdentifier = null;

            if (!TypeIdentifier.TryParse(hierarchyGUSID, out TypeIdentifier? hierarchyTypeIdentifier))
            {
                return false;
            }

            hierarchyIdentifier = new HierarchyIdentifier(hierarchyTypeIdentifier);
            return true;
        }
#nullable disable
        #endregion

        #region Overrides
        public override string ToString()
        {
            return GUSID;
        }

        public override bool Equals(object obj)
        {
            if (obj is HierarchyIdentifier)
            {
                return Equals((HierarchyIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(HierarchyIdentifier otherHierarchyIdentifier)
        {
            return otherHierarchyIdentifier.GUSID.Equals(this.GUSID);
        }

        public override int GetHashCode()
        {
            return GUSID.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(HierarchyIdentifier hierarchyIdentifier1, HierarchyIdentifier hierarchyIdentifier2)
        {
            return hierarchyIdentifier1.Equals(hierarchyIdentifier2);
        }

        public static bool operator !=(HierarchyIdentifier hierarchyIdentifier1, HierarchyIdentifier hierarchyIdentifier2)
        {
            return !hierarchyIdentifier1.Equals(hierarchyIdentifier2);
        }

        public static implicit operator string(HierarchyIdentifier hierarchyIdentifier)
        {
            return hierarchyIdentifier.GUSID;
        }

#nullable enable
        public static implicit operator HierarchyIdentifier?(string hierarchy)
        {
            if (TryParse(hierarchy, out HierarchyIdentifier? hierarchyIdentifier))
            {
                return hierarchyIdentifier;
            }
            else
            {
                throw new ArgumentException($"The string '{hierarchy}' is not a valid hierarchy.");
            }
        }
#nullable disable
        #endregion
    }
}
