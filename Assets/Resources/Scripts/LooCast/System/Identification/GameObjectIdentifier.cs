using System;
using UnityEngine;

namespace LooCast.System.Identification
{
    [Serializable]
    public struct GameObjectIdentifier : IIdentifier
    {
        #region Properties
        public string GUSID
        {
            get
            {
                return $"{TypeIdentifier}({InstanceGUID})";
            }
        }
        public TypeIdentifier TypeIdentifier => typeIdentifier;
        public Guid InstanceGUID => instanceGUID;
        #endregion

        #region Fields
        [SerializeField] private TypeIdentifier typeIdentifier;
        [SerializeField] private Guid instanceGUID;
        #endregion

        #region Constructors
        public GameObjectIdentifier(TypeIdentifier typeIdentifier, Guid instanceGUID = new Guid())
        {
            this.typeIdentifier = typeIdentifier;
            this.instanceGUID = instanceGUID;
        }
        #endregion

        #region Static Methods
        public static bool TryParse(string gusid, out GameObjectIdentifier? gameObjectIdentifier)
        {
            gameObjectIdentifier = null;

            string[] parts = gusid.Split(new char[] { '(', ')' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string typeIdentifierString = parts[0];
            string instanceGUIDString = parts[1];

            if (!TypeIdentifier.TryParse(typeIdentifierString, out TypeIdentifier? typeIdentifier))
            {
                return false;
            }

            if (!Guid.TryParse(instanceGUIDString, out Guid instanceGUID))
            {
                return false;
            }

            gameObjectIdentifier = new GameObjectIdentifier(typeIdentifier.Value, instanceGUID);
            return true;
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is GameObjectIdentifier)
            {
                return Equals((GameObjectIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(GameObjectIdentifier otherGameObjectIdentifier)
        {
            return otherGameObjectIdentifier.GUSID.Equals(this.GUSID);
        }

        public override int GetHashCode()
        {
            return GUSID.GetHashCode();
        }

        public override string ToString()
        {
            return GUSID;
        }
        #endregion

        #region Operators
        public static bool operator ==(GameObjectIdentifier gameObjectIdentifier1, GameObjectIdentifier gameObjectIdentifier2)
        {
            return gameObjectIdentifier1.Equals(gameObjectIdentifier2);
        }

        public static bool operator !=(GameObjectIdentifier gameObjectIdentifier1, GameObjectIdentifier gameObjectIdentifier2)
        {
            return !gameObjectIdentifier1.Equals(gameObjectIdentifier2);
        }

        public static implicit operator string(GameObjectIdentifier gameObjectIdentifier)
        {
            return gameObjectIdentifier.GUSID;
        }

        public static implicit operator GameObjectIdentifier(string gusid)
        {
            if (TryParse(gusid, out GameObjectIdentifier? gameObjectIdentifier))
            {
                return gameObjectIdentifier.Value;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid GUSID.");
            }
        }
        #endregion
    }
}
