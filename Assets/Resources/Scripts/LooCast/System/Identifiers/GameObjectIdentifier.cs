using System;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public class GameObjectIdentifier : Identifier
    {
        #region Properties
        public TypeIdentifier GameObjectTypeIdentifier => gameObjectTypeIdentifier;
        public Guid GameObjectInstanceGUID => gameObjectInstanceGUID;
        #endregion

        #region Fields
        [SerializeField] private readonly TypeIdentifier gameObjectTypeIdentifier;
        [SerializeField] private readonly Guid gameObjectInstanceGUID;
        #endregion

        #region Constructors
        public GameObjectIdentifier(TypeIdentifier gameObjectTypeIdentifier, Guid gameObjectinstanceGUID, string gusid = null) : base(gusid == null ? $"{gameObjectTypeIdentifier}({gameObjectinstanceGUID})" : gusid)
        {
            this.gameObjectTypeIdentifier = gameObjectTypeIdentifier;
            this.gameObjectInstanceGUID = gameObjectinstanceGUID;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusid, out GameObjectIdentifier? gameObjectIdentifier)
        {
            gameObjectIdentifier = null;

            string[] parts = gusid.Split(new char[] { '(', ')' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string gameObjectTypeIdentifierString = parts[0];
            string gameObjectInstanceGUIDString = parts[1];

            if (!TypeIdentifier.TryParse(gameObjectTypeIdentifierString, out TypeIdentifier? gameObjectTypeIdentifier))
            {
                return false;
            }

            if (!Guid.TryParse(gameObjectInstanceGUIDString, out Guid gameObjectinstanceGUID))
            {
                return false;
            }

            gameObjectIdentifier = new GameObjectIdentifier(gameObjectTypeIdentifier, gameObjectinstanceGUID);
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
            if (TryParse(gusid, out GameObjectIdentifier gameObjectIdentifier))
            {
                return gameObjectIdentifier;
            }
            else
            {
                throw new ArgumentException($"The string '{gusid}' is not a valid ContainingGameObject GUSID.");
            }
        }
        #endregion
    }
}
