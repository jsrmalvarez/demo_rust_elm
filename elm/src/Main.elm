port module Main exposing (main)

import Browser
import File exposing (File)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode as JD
import Task

-- MODEL

type alias MyFile =
    { buff : File, content : String }


type alias Model =
    { files : List MyFile }


initialModel : () -> ( Model, Cmd msg )
initialModel _ =
    ( { files = [] }, Cmd.none )


type Msg
    = ChooseFiles (List File)
    | Read
    | FileReadSuccess String String
    | Send String

-- UPDATE

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Send m ->
            ( model, sendMessage m )

        ChooseFiles files ->
            { model | files = List.map (\file -> MyFile file "") files } |> update Read

        Read ->
            ( model
            , List.map
                (\file ->
                    File.toString file.buff |> Task.perform (FileReadSuccess (File.name file.buff))
                )
                model.files
                |> Cmd.batch
            )

        FileReadSuccess name content ->
            ( { model
                | files =
                    List.map
                        (\file ->
                            if File.name file.buff == name then
                                { file | content = content }

                            else
                                file
                        )
                        model.files
              }
            , Cmd.none
            )


view : Model -> Html Msg
view model =
    div []
        [ input
            [ type_ "file"
            , multiple False
            , on "change" (JD.map ChooseFiles filesDecoder)
            ]
            []
        , button [ onClick Read ] [ text "Read" ]
        , button [ onClick (Send "hola desde Elm") ] [ text "Test Outgoing Port" ]
        , div [] <|
            List.map
                (\file ->
                    div []
                        [ p [] [ text (File.name file.buff) ]
                        , text file.content
                        , hr [] []
                        ]
                )
                model.files
        ]


filesDecoder =
    JD.at [ "target", "files" ] (JD.list File.decoder)

-- PORTS


port sendMessage : String -> Cmd msg

{--
port messageReceiver : (String -> msg) -> Sub msg

-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions _ =
  messageReceiver Recv
--}

-- MAIN

main : Program () Model Msg
main =
    Browser.element
        { init = initialModel
        , view = view
        , update = update
        , subscriptions = \_ -> Sub.none
        }
