import oauth2, {OAuthAuthCodeRepository, OAuthClient, OAuthScope, OAuthScopeRepository, OAuthToken, OAuthTokenRepository, OAuthUserRepository} from "@jmondi/oauth2-server";
class Client implements oauth2.OAuthClient
{
    id : string;
    name : string;
    redirectUris: string[];
    allowedGrants: oauth2.GrantIdentifier[];
    scopes: oauth2.OAuthScope[];
    secret?: string | null | undefined;
    constructor(n_name : string, n_secret : string | null | undefined, n_uri: string, n_id : string, n_scope : string)
    {
        this.name = n_name;
        this.id = n_id;
        this.secret = n_secret;
        this.redirectUris = new Array(n_uri);
        this.allowedGrants = new Array("authorization_code");
        this.scopes = new Array( new ScopeSet("TODO"));

    }

}

class ScopeSet implements OAuthScope
{
    name : string;
    constructor(n_name : string)
    {
        this.name = n_name;
    }
} 

class ScopeRepo implements OAuthScopeRepository
{
    getAllByIdentifiers(scopeNames: string[]): Promise<oauth2.OAuthScope[]> {
        throw new Error("Method not implemented.");
    }
    finalize(scopes: oauth2.OAuthScope[], identifier: oauth2.GrantIdentifier, client: oauth2.OAuthClient, user_id?: oauth2.OAuthUserIdentifier | undefined): Promise<oauth2.OAuthScope[]> {
        throw new Error("Method not implemented.");
    }
    
}

class ClientProvider implements oauth2.OAuthClientRepository
{

    clients : OAuthClient[];

    async addClient(client : OAuthClient) {
        await this.getByIdentifier(client.id).catch(() => this.clients.push(client));
        
    }

    async getByIdentifier(clientId: string): Promise<oauth2.OAuthClient> {

        for(const client of this.clients)
        {
            if(client.id === clientId)
            {
                return client;
            }

        }

        throw new Error("Client not found!"); // I hate Javascript 

    }
    isClientValid(grantType: oauth2.GrantIdentifier, client: oauth2.OAuthClient, clientSecret?: string | undefined): Promise<boolean> {
        throw new Error("Method not implemented.");
    }

    constructor()
    {
        this.clients = new Array();
    }
}

class AuthCodeRepo implements OAuthAuthCodeRepository
{
    getByIdentifier(authCodeCode: string): Promise<oauth2.OAuthAuthCode> {
        throw new Error("Method not implemented.");
    }
    issueAuthCode(client: oauth2.OAuthClient, user: oauth2.OAuthUser | undefined, scopes: oauth2.OAuthScope[]): oauth2.OAuthAuthCode | Promise<oauth2.OAuthAuthCode> {
        throw new Error("Method not implemented.");
    }
    persist(authCode: oauth2.OAuthAuthCode): Promise<void> {
        throw new Error("Method not implemented.");
    }
    isRevoked(authCodeCode: string): Promise<boolean> {
        throw new Error("Method not implemented.");
    }
    revoke(authCodeCode: string): Promise<void> {
        throw new Error("Method not implemented.");
    }
    
}

class UserRepo implements OAuthUserRepository
{
    getUserByCredentials(identifier: oauth2.OAuthUserIdentifier, password?: string | undefined, grantType?: oauth2.GrantIdentifier | undefined, client?: oauth2.OAuthClient | undefined): Promise<oauth2.OAuthUser | undefined> {
        throw new Error("Method not implemented.");
    }
    
}

class TokenRepo implements OAuthTokenRepository
{
    issueToken(client: oauth2.OAuthClient, scopes: oauth2.OAuthScope[], user?: oauth2.OAuthUser | null | undefined): Promise<oauth2.OAuthToken> {
        throw new Error("Method not implemented.");
    }
    issueRefreshToken(accessToken: oauth2.OAuthToken, client: oauth2.OAuthClient): Promise<oauth2.OAuthToken> {
        throw new Error("Method not implemented.");
    }
    persist(accessToken: oauth2.OAuthToken): Promise<void> {
        throw new Error("Method not implemented.");
    }
    revoke(accessToken: oauth2.OAuthToken): Promise<void> {
        throw new Error("Method not implemented.");
    }
    revokeDescendantsOf?(authCodeId: string): Promise<void> {
        throw new Error("Method not implemented.");
    }
    isRefreshTokenRevoked(refreshToken: oauth2.OAuthToken): Promise<boolean> {
        throw new Error("Method not implemented.");
    }
    getByRefreshToken(refreshTokenToken: string): Promise<oauth2.OAuthToken> {
        throw new Error("Method not implemented.");
    }

}

class Token implements OAuthToken
{
    accessToken: string;
    accessTokenExpiresAt: Date;
    refreshToken?: string | null | undefined;
    refreshTokenExpiresAt?: Date | null | undefined;
    client: oauth2.OAuthClient;
    user?: oauth2.OAuthUser | null | undefined;
    scopes: oauth2.OAuthScope[];
    originatingAuthCodeId?: string | undefined;

}

