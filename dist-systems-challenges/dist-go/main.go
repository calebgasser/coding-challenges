package main

import (
	"encoding/json"
	"golang.org/x/exp/slices"
	"log"

	"github.com/google/uuid"
	maelstrom "github.com/jepsen-io/maelstrom/demo/go"
)

func main() {
	n := maelstrom.NewNode()
	allNodes := make([]float64, 0)
	n.Handle("echo", func(msg maelstrom.Message) error {
		var body map[string]any
		if err := json.Unmarshal(msg.Body, &body); err != nil {
			return err
		}
		body["type"] = "echo_ok"
		return n.Reply(msg, body)
	})
	n.Handle("generate", func(msg maelstrom.Message) error {
		var body map[string]any
		if err := json.Unmarshal(msg.Body, &body); err != nil {
			return err
		}
		body["type"] = "generate_ok"
		body["id"] = uuid.New().String()
		return n.Reply(msg, body)
	})
	n.Handle("broadcast", func(msg maelstrom.Message) error {
		var body map[string]any
		if err := json.Unmarshal(msg.Body, &body); err != nil {
			return err
		}
		body["type"] = "broadcast_ok"
		messageId := body["message"].(float64)
		if !slices.Contains(allNodes, messageId) {
			allNodes = append(allNodes, messageId)
		}
		return n.Reply(msg, body)
	})
	n.Handle("read", func(msg maelstrom.Message) error {
		var body map[string]any
		if err := json.Unmarshal(msg.Body, &body); err != nil {
			return err
		}
		body["type"] = "read_ok"
		body["messages"] = allNodes

		return n.Reply(msg, body)
	})
	n.Handle("topology", func(msg maelstrom.Message) error {
		var body map[string]any
		if err := json.Unmarshal(msg.Body, &body); err != nil {
			return err
		}
		resBody := map[string]any{
			"type": "topology_ok",
		}
		return n.Reply(msg, resBody)
	})
	if err := n.Run(); err != nil {
		log.Fatal(err)
	}
}
